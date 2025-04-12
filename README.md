# Siren

An Indonesian serverless V2Ray tunnel solution designed for simplicity and efficiency.

## Features

- **Protocol Support:**
  - [x] **Vmess**
  - [x] **Trojan**
  - [x] **VLESS**
  - [x] **Shadowsocks** 
- **Domain over HTTPS** – Ensuring encrypted and safe communication.

## Endpoints

- **`/`** 
- **`/link`**
- **`/sub`**

## Deployment

### CI Deployment with GitHub Actions

Follow these steps to deploy Siren using **GitHub Actions** and Cloudflare Workers:

1. **Create an API Token:**  
   Head to [Cloudflare API Token Creation](https://developers.cloudflare.com/fundamentals/api/get-started/create-token/) and generate an API token.

2. **Store the API Token:**  
   In your GitHub repository, create a **Repository Secret** named `CLOUDFLARE_API_TOKEN` and paste the token you generated in the previous step.

3. **Enable Workflows:**  
   Go to the **Actions** tab in your GitHub repository and enable workflows.

4. **Deploy:**  
   Push a commit or manually trigger the workflow to start the deployment process.

5. **Access Your Deployment:**  
   Once the workflow completes, you can access your deployed service at:  
   `https://YOUR-WORKERS-SUBDOMAIN.workers.dev`

## Credits

This project is based on [FoolVPN-ID](https://github.com/FoolVPN-ID/).

