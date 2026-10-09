//! 内置静态服务：`danqing serve`
//!
//! 用来替代 `python -m http.server 3788` 预览 `showcase/index.html`。
//! 只用标准库，只绑 127.0.0.1——mirrored 网络环境下绑 0.0.0.0 等于把本地目录
//! 暴露到局域网，这不是一个预览命令该有的副作用。

use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;
use std::path::{Path, PathBuf};

pub fn run(root: &Path, port: u16) -> Result<(), String> {
    let addr = format!("127.0.0.1:{port}");
    let listener = TcpListener::bind(&addr).map_err(|e| format!("无法监听 {addr}：{e}"))?;
    println!("丹青配色参考页：http://{addr}/showcase/");
    println!("按 Ctrl+C 停止。");

    for stream in listener.incoming() {
        match stream {
            Ok(mut conn) => {
                if let Err(e) = handle(&mut conn, root) {
                    eprintln!("[warn] 连接处理失败：{e}");
                }
            }
            Err(e) => eprintln!("[warn] 接受连接失败：{e}"),
        }
    }
    Ok(())
}

fn handle(conn: &mut std::net::TcpStream, root: &Path) -> Result<(), String> {
    let peer = conn.peer_addr().ok();
    let mut reader = BufReader::new(conn.try_clone().map_err(|e| e.to_string())?);

    let mut request_line = String::new();
    if reader.read_line(&mut request_line).map_err(|e| e.to_string())? == 0 {
        return Ok(());
    }
    // 丢弃请求头
    loop {
        let mut line = String::new();
        if reader.read_line(&mut line).map_err(|e| e.to_string())? == 0 {
            break;
        }
        if line.trim().is_empty() {
            break;
        }
    }

    let parts: Vec<&str> = request_line.split_whitespace().collect();
    if parts.len() < 2 {
        return Ok(());
    }
    let (method, target) = (parts[0], parts[1]);
    if method != "GET" && method != "HEAD" {
        let _ = conn.write_all(b"HTTP/1.1 405 Method Not Allowed\r\nContent-Length: 0\r\nConnection: close\r\n\r\n");
        return Ok(());
    }

    let raw = target.split('?').next().unwrap_or("/");
    // 先解码再校验：不解码的话含空格的路径会被当成字面量取不到；解码后再交给
    // safe_join（拒绝 `..` 段 + canonicalize 前缀校验），两种绕边界的方式都堵住。
    let decoded = percent_decode(raw.trim_start_matches('/'));
    let rel = if decoded.is_empty() {
        "showcase/index.html".to_string()
    } else {
        decoded
    };
    let mut path = safe_join(root, &rel);

    // 目录请求回落 index.html：`/showcase/` 与 `/` 都该看到展示页，而不是 404——
    // 文档与启动提示里给出的就是带斜杠的那个地址。
    if let Some(p) = &path {
        if p.is_dir() {
            let dir_rel = rel.trim_end_matches('/');
            path = safe_join(root, &format!("{dir_rel}/index.html"));
        }
    }
    let _ = peer;

    let Some(path) = path else {
        return send_simple(conn, 403, "Forbidden");
    };
    // 落到根目录之外 → 403；压根不存在 → 404。两者都不是「读到了」，分开报让排查更快。
    if !path.exists() {
        return send_simple(conn, 404, "Not Found");
    }
    if within_root(&path, root).is_err() {
        return send_simple(conn, 403, "Forbidden");
    }

    match fs::read(&path) {
        Ok(body) => {
            let ctype = mime(&path);
            let head = format!(
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: {}\r\nCache-Control: no-store\r\nConnection: close\r\n\r\n",
                body.len(),
                ctype
            );
            let _ = conn.write_all(head.as_bytes());
            if method == "GET" {
                let _ = conn.write_all(&body);
            }
        }
        Err(_) => return send_simple(conn, 404, "Not Found"),
    }
    let _ = conn.flush();
    Ok(())
}

/// `%XX` 解码。大小写十六进制都认；非法序列按字面保留（随后会被过滤或 404）。
fn percent_decode(s: &str) -> String {
    let bytes = s.as_bytes();
    let mut out: Vec<u8> = Vec::with_capacity(bytes.len());
    let mut i = 0usize;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            if let (Some(hi), Some(lo)) = (hex_val(bytes[i + 1]), hex_val(bytes[i + 2])) {
                out.push(hi * 16 + lo);
                i += 3;
                continue;
            }
        }
        out.push(bytes[i]);
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

fn hex_val(c: u8) -> Option<u8> {
    match c {
        b'0'..=b'9' => Some(c - b'0'),
        b'a'..=b'f' => Some(c - b'a' + 10),
        b'A'..=b'F' => Some(c - b'A' + 10),
        _ => None,
    }
}

/// 把 URL 路径拼到根目录。**不做**存在性判断，只负责拼出候选路径。
fn safe_join(root: &Path, rel: &str) -> Option<PathBuf> {
    if rel.contains('\0') {
        return None;
    }
    let mut out = root.to_path_buf();
    for seg in rel.split('/') {
        if seg.is_empty() || seg == "." {
            continue;
        }
        if seg == ".." {
            return None;
        }
        out.push(seg);
    }
    Some(out)
}

/// 候选路径是否落在根目录之内（先把 `..` 折叠掉，避免符号链接绕过）。
fn within_root(path: &Path, root: &Path) -> Result<(), ()> {
    let (Ok(p), Ok(r)) = (fs::canonicalize(path), fs::canonicalize(root)) else {
        return Err(());
    };
    if p.starts_with(&r) { Ok(()) } else { Err(()) }
}

fn send_simple(conn: &mut std::net::TcpStream, code: u16, text: &str) -> Result<(), String> {
    let body = format!("{code} {text}");
    let head = format!(
        "HTTP/1.1 {code} {text}\r\nContent-Length: {}\r\nContent-Type: text/plain; charset=utf-8\r\nConnection: close\r\n\r\n",
        body.len()
    );
    conn.write_all(head.as_bytes()).map_err(|e| e.to_string())?;
    conn.write_all(body.as_bytes()).map_err(|e| e.to_string())?;
    Ok(())
}

fn mime(path: &Path) -> &'static str {
    match path.extension().and_then(|e| e.to_str()) {
        Some("html") | Some("htm") => "text/html; charset=utf-8",
        Some("css") => "text/css; charset=utf-8",
        Some("js") => "text/javascript; charset=utf-8",
        Some("json") => "application/json; charset=utf-8",
        Some("svg") => "image/svg+xml",
        Some("png") => "image/png",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("webp") => "image/webp",
        Some("md") => "text/markdown; charset=utf-8",
        Some("txt") => "text/plain; charset=utf-8",
        _ => "application/octet-stream",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn percent_decode_roundtrip() {
        assert_eq!(percent_decode("a%20b"), "a b");
        assert_eq!(percent_decode("%2e%2e"), "..");
        assert_eq!(percent_decode("丹"), "丹");
    }

    #[test]
    fn traversal_is_rejected() {
        let root = PathBuf::from(".");
        assert!(safe_join(&root, "tokens/source.json").is_some());
        assert!(safe_join(&root, "../secrets").is_none());
        assert!(safe_join(&root, "a/../../b").is_none());
    }
}
