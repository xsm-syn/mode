mod common;
mod config;
mod proxy;

use crate::config::Config;
use crate::proxy::*;

use std::collections::HashMap;
use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use serde::Serialize;
use serde_json::json;
use uuid::Uuid;
use worker::*;
use once_cell::sync::Lazy;
use regex::Regex;

static PROXYIP_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"^.+-\d+$").unwrap());

#[event(fetch)]
async fn main(req: Request, env: Env, _: Context) -> Result<Response> {
    let uuid = env
        .var("UUID")
        .map(|x| Uuid::parse_str(&x.to_string()).unwrap_or_default())?;
    let host = req.url()?.host().map(|x| x.to_string()).unwrap_or_default();
    let main_page_url = env.var("MAIN_PAGE_URL").map(|x|x.to_string()).unwrap();
    let sub_page_url = env.var("SUB_PAGE_URL").map(|x|x.to_string()).unwrap();
    let config = Config { uuid, host: host.clone(), proxy_addr: host, proxy_port: 443, main_page_url, sub_page_url};

    Router::with_data(config)
        .on_async("/", fe)
        .on_async("/sub", sub)
        .on("/link", link)
        .on_async("/:proxyip", tunnel)
        .on_async("/Inconigto-Mode/:proxyip", tunnel)
        .run(req, env)
        .await
}

async fn get_response_from_url(url: String) -> Result<Response> {
    let req = Fetch::Url(Url::parse(url.as_str())?);
    let mut res = req.send().await?;
    Response::from_html(res.text().await?)
}

async fn fe(_: Request, cx: RouteContext<Config>) -> Result<Response> {
    get_response_from_url(cx.data.main_page_url).await
}

async fn sub(_: Request, cx: RouteContext<Config>) -> Result<Response> {
    get_response_from_url(cx.data.sub_page_url).await
}


async fn tunnel(req: Request, mut cx: RouteContext<Config>) -> Result<Response> {
    let mut proxyip = cx.param("proxyip").unwrap().to_string();
    if proxyip.len() == 2 {
        let req = Fetch::Url(Url::parse("https://raw.githubusercontent.com/FoolVPN-ID/Nautica/refs/heads/main/kvProxyList.json")?);
        let mut res = req.send().await?;
        if res.status_code() == 200 {
            let proxy_kv: HashMap<String, Vec<String>> = serde_json::from_str(&res.text().await?)?;
            proxyip = proxy_kv[&proxyip][0].clone().replace(":", "-");
        }
    }

    if PROXYIP_PATTERN.is_match(&proxyip) {
        if let Some((addr, port_str)) = proxyip.split_once('-') {
            if let Ok(port) = port_str.parse() {
                cx.data.proxy_addr = addr.to_string();
                cx.data.proxy_port = port;
            }
        }
    }
    
    let upgrade = req.headers().get("Upgrade")?.unwrap_or("".to_string());
    if upgrade == "websocket".to_string() {
        let WebSocketPair { server, client } = WebSocketPair::new()?;
        server.accept()?;
    
        wasm_bindgen_futures::spawn_local(async move {
            let events = server.events().unwrap();
            if let Err(e) = ProxyStream::new(cx.data, &server, events).process().await {
                console_log!("[tunnel]: {}", e);
            }
        });
    
        Response::from_websocket(client)
    } else {
        Response::from_html("https://inconigto-mode.web.id/")
    }

}

fn link(_: Request, cx: RouteContext<Config>) -> Result<Response> {
    // Extract host from context data
    let host = cx.data.host.to_string();
    
    // Create an HTML response string with basic structure - using only one host parameter
    let html = format!(
        r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <meta name="description" content="Create V2ray accounts with VLESS, TROJAN, and SHADOWSHOCK protocols using Cloudflare Workers">
    <meta name="keywords" content="V2ray, Cloudflare Workers, VLESS, TROJAN, SHADOWSHOCK, proxy, VPN">
    <meta name="author" content="XSM">
    <meta name="robots" content="index, follow">
    
    <!-- Open Graph / Facebook -->
    <meta property="og:type" content="website">
    <meta property="og:url" content="https://{host}/">
    <meta property="og:title" content="XSM - V2ray Account Generator">
    <meta property="og:description" content="Create V2ray accounts with VLESS, TROJAN, and SHADOWSHOCK protocols using Cloudflare Workers">
    <meta property="og:image" content="https://raw.githubusercontent.com/akulelaki696/bg/refs/heads/main/20250106_010158.jpg">
    <meta property="og:image:width" content="1200">
    <meta property="og:image:height" content="630">
    <meta property="og:site_name" content="XSM">
    
    <!-- Twitter -->
    <meta property="twitter:card" content="summary_large_image">
    <meta property="twitter:url" content="https://{host}/">
    <meta property="twitter:title" content="XSM - V2ray Account Generator">
    <meta property="twitter:description" content="Create V2ray accounts with VLESS, TROJAN, and SHADOWSHOCK protocols using Cloudflare Workers">
    <meta property="twitter:image" content="https://raw.githubusercontent.com/akulelaki696/bg/refs/heads/main/20250106_010158.jpg">
    <meta property="twitter:creator" content="@InconigtoMode">
	
    <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.4.0/css/all.min.css">    
    <meta name="theme-color" content="#0f0c29"> 
    <!-- Favicon -->
    <link rel="icon" href="https://raw.githubusercontent.com/AFRcloud/BG/main/icons8-film-noir-80.png"/>
    
    <title>XSM</title>
    <!-- Theme Color for browsers -->
     
    <style>
        @import url('https://fonts.googleapis.com/css2?family=Rajdhani:wght@500;600;700&family=Orbitron:wght@400;500;700&display=swap');
        
        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }
        
        body {
            font-family: 'Rajdhani', sans-serif;
            background: linear-gradient(135deg, #0f0c29, #302b63, #24243e);
            color: #fff;
            min-height: 100vh;
            display: flex;
            justify-content: center;
            align-items: center;
            padding: 20px;
            overflow-x: hidden;
        }
        
        .container {
            position: relative;
            width: 100%;
            max-width: 420px;
        }
        
        .card {
            background: rgba(15, 14, 32, 0.8);
            border-radius: 20px;
            backdrop-filter: blur(10px);
            border: 1px solid rgba(255, 255, 255, 0.1);
            box-shadow: 0 15px 35px rgba(0, 0, 0, 0.5);
            overflow: hidden;
            padding: 40px 30px;
            position: relative;
            z-index: 1;
        }
        
        .card::before {
            content: '';
            position: absolute;
            top: -50%;
            left: -50%;
            width: 200%;
            height: 200%;
            background: linear-gradient(
                45deg,
                transparent 0%,
                rgba(78, 78, 219, 0.1) 45%,
                rgba(78, 78, 219, 0.3) 50%,
                rgba(78, 78, 219, 0.1) 55%,
                transparent 100%
            );
            z-index: -1;
            transform: rotate(30deg);
            animation: shine 6s infinite linear;
        }
        
        @keyframes shine {
            0% {
                transform: translateY(100%) rotate(30deg);
            }
            100% {
                transform: translateY(-100%) rotate(30deg);
            }
        }
        
        .profile-header {
            display: flex;
            flex-direction: column;
            align-items: center;
            margin-bottom: 30px;
            position: relative;
        }
        
        .profile-img-container {
            position: relative;
            width: 150px;
            height: 150px;
            margin-bottom: 25px;
        }
        
        .profile-img {
            width: 100%;
            height: 100%;
            border-radius: 50%;
            object-fit: cover;
            border: 3px solid transparent;
            background: linear-gradient(145deg, #6a11cb, #2575fc) border-box;
            animation: pulse 3s infinite;
            box-shadow: 
                0 0 15px rgba(106, 17, 203, 0.5),
                0 0 30px rgba(106, 17, 203, 0.3),
                0 0 45px rgba(106, 17, 203, 0.1);
        }
        
        @keyframes pulse {
            0% {
                box-shadow: 
                    0 0 15px rgba(106, 17, 203, 0.5),
                    0 0 30px rgba(106, 17, 203, 0.3),
                    0 0 45px rgba(106, 17, 203, 0.1);
            }
            50% {
                box-shadow: 
                    0 0 20px rgba(106, 17, 203, 0.7),
                    0 0 40px rgba(106, 17, 203, 0.5),
                    0 0 60px rgba(106, 17, 203, 0.3);
            }
            100% {
                box-shadow: 
                    0 0 15px rgba(106, 17, 203, 0.5),
                    0 0 30px rgba(106, 17, 203, 0.3),
                    0 0 45px rgba(106, 17, 203, 0.1);
            }
        }
        
        .profile-img-container::after {
            content: '';
            position: absolute;
            top: -10px;
            left: -10px;
            right: -10px;
            bottom: -10px;
            border-radius: 50%;
            background: linear-gradient(45deg, #6a11cb, #2575fc, #6a11cb);
            z-index: -1;
            opacity: 0.5;
            animation: rotate 10s linear infinite;
        }
        
        @keyframes rotate {
            0% {
                transform: rotate(0deg);
            }
            100% {
                transform: rotate(360deg);
            }
        }
        
        .profile-name {
            font-family: 'Orbitron', sans-serif;
            font-size: 2rem;
            font-weight: 700;
            margin-bottom: 5px;
            text-align: center;
            background: linear-gradient(to right, #6a11cb, #2575fc);
            -webkit-background-clip: text;
            background-clip: text;
            -webkit-text-fill-color: transparent;
            text-shadow: 0 2px 10px rgba(106, 17, 203, 0.3);
        }
        
        .service-description {
            font-size: 1rem;
            color: #a0a0ff;
            margin-bottom: 15px;
            text-align: center;
            max-width: 280px;
            margin-left: auto;
            margin-right: auto;
        }
        
        .status-badge {
            background: linear-gradient(45deg, #11998e, #38ef7d);
            color: #fff;
            padding: 5px 15px;
            border-radius: 20px;
            font-size: 0.9rem;
            font-weight: 600;
            margin-bottom: 20px;
            box-shadow: 0 5px 15px rgba(56, 239, 125, 0.3);
            display: flex;
            align-items: center;
            gap: 8px;
        }
        
        .status-badge i {
            font-size: 0.8rem;
        }
        
        .divider {
            width: 80%;
            height: 2px;
            background: linear-gradient(to right, transparent, rgba(106, 17, 203, 0.5), transparent);
            margin: 20px auto;
        }
        
        .profile-links {
            display: flex;
            flex-direction: column;
            gap: 15px;
            margin-bottom: 30px;
        }
        
        .profile-link {
            display: flex;
            align-items: center;
            gap: 15px;
            padding: 12px 20px;
            background: rgba(255, 255, 255, 0.05);
            border-radius: 12px;
            transition: all 0.3s ease;
            text-decoration: none;
            color: #fff;
            border-left: 3px solid #6a11cb;
        }
        
        .profile-link:hover {
            background: rgba(255, 255, 255, 0.1);
            transform: translateX(5px);
            box-shadow: 0 5px 15px rgba(0, 0, 0, 0.2);
        }
        
        .profile-link i {
            font-size: 1.2rem;
            color: #a0a0ff;
        }
        
        .profile-link span {
            font-size: 1rem;
            word-break: break-all;
        }
        
        .action-button {
            display: block;
            width: 100%;
            padding: 15px;
            background: linear-gradient(45deg, #6a11cb, #2575fc);
            border: none;
            border-radius: 12px;
            color: white;
            font-family: 'Rajdhani', sans-serif;
            font-size: 1.1rem;
            font-weight: 600;
            cursor: pointer;
            transition: all 0.3s ease;
            text-align: center;
            text-decoration: none;
            letter-spacing: 1px;
            box-shadow: 0 10px 20px rgba(106, 17, 203, 0.3);
            position: relative;
            overflow: hidden;
        }
        
        .action-button::before {
            content: '';
            position: absolute;
            top: 0;
            left: -100%;
            width: 100%;
            height: 100%;
            background: linear-gradient(
                90deg,
                transparent,
                rgba(255, 255, 255, 0.2),
                transparent
            );
            transition: 0.5s;
        }
        
        .action-button:hover::before {
            left: 100%;
        }
        
        .action-button:hover {
            transform: translateY(-3px);
            box-shadow: 0 15px 25px rgba(106, 17, 203, 0.4);
        }
        
        .action-button:active {
            transform: translateY(0);
        }
        
        .protocol-icons {
            display: flex;
            justify-content: center;
            gap: 15px;
            margin: 20px 0;
        }
        
        .protocol-icon {
            display: flex;
            flex-direction: column;
            align-items: center;
            gap: 5px;
        }
        
        .protocol-icon-circle {
            width: 50px;
            height: 50px;
            border-radius: 50%;
            background: rgba(255, 255, 255, 0.05);
            display: flex;
            align-items: center;
            justify-content: center;
            color: #a0a0ff;
            font-size: 1.2rem;
            border: 1px solid rgba(255, 255, 255, 0.1);
            transition: all 0.3s ease;
        }
        
        .protocol-icon-text {
            font-size: 0.8rem;
            color: #a0a0ff;
        }
        
        .social-icons {
            display: flex;
            justify-content: center;
            gap: 20px;
            margin-top: 30px;
        }
        
        .social-icon {
            width: 40px;
            height: 40px;
            border-radius: 50%;
            background: rgba(255, 255, 255, 0.05);
            display: flex;
            align-items: center;
            justify-content: center;
            color: #a0a0ff;
            text-decoration: none;
            transition: all 0.3s ease;
            border: 1px solid rgba(255, 255, 255, 0.1);
        }
        
        .social-icon:hover {
            transform: translateY(-5px);
            background: linear-gradient(45deg, #6a11cb, #2575fc);
            color: white;
            box-shadow: 0 5px 15px rgba(106, 17, 203, 0.3);
        }
        
        .footer-credit {
            margin-top: 40px;
            text-align: center;
            font-size: 0.8rem;
            color: rgba(255, 255, 255, 0.5);
            line-height: 1.5;
        }
        
        .footer-credit a {
            color: #a0a0ff;
            text-decoration: none;
            transition: color 0.3s ease;
        }
        
        .footer-credit a:hover {
            color: #ffffff;
            text-decoration: underline;
        }
        
        .footer-divider {
            width: 50%;
            height: 1px;
            background: linear-gradient(to right, transparent, rgba(106, 17, 203, 0.3), transparent);
            margin: 15px auto;
        }
        
        .particles {
            position: fixed;
            top: 0;
            left: 0;
            width: 100%;
            height: 100%;
            z-index: -2;
            overflow: hidden;
        }
        
        .particle {
            position: absolute;
            width: 5px;
            height: 5px;
            background: rgba(255, 255, 255, 0.5);
            border-radius: 50%;
            animation: float 15s infinite linear;
        }
        
        @keyframes float {
            0% {
                transform: translateY(0) rotate(0deg);
                opacity: 1;
                border-radius: 0;
            }
            100% {
                transform: translateY(-1000px) rotate(720deg);
                opacity: 0;
                border-radius: 50%;
            }
        }
    </style>
</head>
<body>
    <div class="particles" id="particles"></div>
    
    <div class="container">
        <div class="card">
            <div class="profile-header">
                <div class="profile-img-container">
                    <img src="https://raw.githubusercontent.com/akulelaki696/bg/refs/heads/main/20250106_010158.jpg" alt="XSM Profile" class="profile-img">
                </div>
                <h1 class="profile-name">XSM</h1>
                <p class="service-description">V2ray Account Generator based on Cloudflare Workers</p>
                <div class="status-badge">
                    <i class="fas fa-circle"></i> Service Online
                </div>
            </div>
            
            <div class="protocol-icons">
                <div class="protocol-icon">
                    <div class="protocol-icon-circle">
                        <i class="fas fa-bolt"></i>
                    </div>
                    <span class="protocol-icon-text">VLESS</span>
                </div>
                <div class="protocol-icon">
                    <div class="protocol-icon-circle">
                        <i class="fas fa-shield-alt"></i>
                    </div>
                    <span class="protocol-icon-text">TROJAN</span>
                </div>
                <div class="protocol-icon">
                    <div class="protocol-icon-circle">
                        <i class="fas fa-ghost"></i>
                    </div>
                    <span class="protocol-icon-text">SHADOW</span>
                </div>
            </div>
            
            <div class="divider"></div>
            
            <div class="profile-links">
                <a href="https://t.me/noir7R" target="_blank" class="profile-link">
                    <i class="fab fa-telegram"></i>
                    <span>@noir7R</span>
                </a>
                <a href="https://t.me/InconigtoMode" target="_blank" class="profile-link">
                    <i class="fab fa-telegram"></i>
                    <span>@InconigtoMode</span>
                </a>
                <a href="https://t.me/Inconigto_Mode" target="_blank" class="profile-link">
                    <i class="fab fa-telegram"></i>
                    <span>@Inconigto_Mode</span>
                </a>
            </div>
            
            <a href="https://{host}" target="_blank" class="action-button">
                CREATE V2RAY ACCOUNT
            </a>
            
            <div class="footer-credit">
                <div class="footer-divider"></div>
                <p>&copy; <script>document.write(new Date().getFullYear())</script> XSM</p>
                <p>Powered by <i class="fas fa-heart" style="color: #ff6b6b;"></i> by <a href="https://t.me/after_sweet" target="_blank">XSM</a> </p>
            </div>
        </div>
    </div>
    
    <script>
        // Create floating particles
        const particlesContainer = document.getElementById("particles");
        const particleCount = 50;
        
        for (let i = 0; i < particleCount; i++) {
            const particle = document.createElement("div");
            particle.classList.add("particle");
            
            // Random position
            const posX = Math.random() * 100;
            const posY = Math.random() * 100;
            
            // Random size
            const size = Math.random() * 5 + 1;
            
            // Random opacity
            const opacity = Math.random() * 0.5 + 0.1;
            
            // Random animation duration
            const duration = Math.random() * 15 + 5;
            
            // Random animation delay
            const delay = Math.random() * 5;
            
            // Random color
            const colors = ["#6a11cb", "#2575fc", "#a0a0ff", "#ffffff"];
            const color = colors[Math.floor(Math.random() * colors.length)];
            
            // Apply styles
            particle.style.left = `${posX}%`;
            particle.style.top = `${posY}%`;
            particle.style.width = `${size}px`;
            particle.style.height = `${size}px`;
            particle.style.opacity = opacity;
            particle.style.background = color;
            particle.style.animationDuration = `${duration}s`;
            particle.style.animationDelay = `${delay}s`;
            
            particlesContainer.appendChild(particle);
        }
    </script>
</body>
</html>
        "#
    );

    // Return HTML response
    Response::from_html(html)
}


    // Return HTML response
    Response::from_html(html)
}
