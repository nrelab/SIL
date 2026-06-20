# SIL API Reference

## Base URL

```
http://localhost:8000
```

## Endpoints

### POST /scan

Scan input text for security risks.

**Request:**
```json
{"input": "раypal"}
```

**Response:**
```json
{
  "input": "раypal",
  "normalized": "paypal",
  "flags": ["VISUAL_MISMATCH_DETECTED"],
  "decision": "BLOCK"
}
```

### GET /health

Health check.

**Response:**
```json
{"status": "ok", "system": "SIL operational"}
```

### GET /metrics

Prometheus metrics endpoint.
