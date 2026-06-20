from __future__ import annotations

import json
import uuid

import redis


class SilQueue:
    def __init__(self, host: str = "redis", port: int = 6379, queue_name: str = "sil_queue"):
        self.client = redis.Redis(host=host, port=port, decode_responses=True)
        self.queue_name = queue_name
        self.result_prefix = "sil_result:"

    def enqueue(self, payload: dict) -> str:
        job_id = str(uuid.uuid4())
        job = {"id": job_id, **payload}
        self.client.rpush(self.queue_name, json.dumps(job))
        return job_id

    def dequeue(self) -> dict | None:
        _, data = self.client.blpop(self.queue_name, timeout=1)
        if data:
            return json.loads(data)
        return None

    def complete(self, job_id: str, result: dict) -> None:
        self.client.set(f"{self.result_prefix}{job_id}", json.dumps(result))
