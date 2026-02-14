# 🚀 NovaCore AI Gateway | Enterprise-Grade Proxy

**NovaCore** is a high-performance AI Gateway built in **Rust**, designed for security, cost-management, and high availability. It acts as a secure bridge between your applications and LLM providers.



## 🌟 Key Features

- **🛡️ PII Redaction:** Automatically detects and masks sensitive data (SSN, Emails, CPFs) before it leaves your infrastructure.
- **🔄 Smart Failover:** High-availability routing. If OpenAI is down, it automatically switches to Google Gemini.
- **🔐 Enterprise Security:** Hardened with `X-API-KEY` header validation.
- **📊 Cost Auditing:** Real-time logging of token usage and estimated costs for business intelligence.
- **⚡ Rust-Powered:** Ultra-low latency and minimal memory footprint.

---

## 🛠️ System Architecture

1. **Authentication:** Validates client credentials via environment-stored secrets.
2. **Privacy Layer:** Scans payload for sensitive information using optimized Regex.
3. **Intent Classifier:** Determines the best model for the specific task.
4. **Resilience:** Circuit breakers prevent system cascading failures.
5. **Auditor:** Logs encrypted metadata for billing and compliance.

---

## 🚀 Getting Started

### 1. Environment Setup
Create a `.env` file in the root directory:
```env
GATEWAY_API_KEY=your_secure_admin_token
OPENAI_KEY=your_openai_api_key
GEMINI_KEY=your_gemini_api_key
PORT=3000