import json
import os
import time

import redis

from core_bridge import run_sil_pipeline

REDIS_HOST = os.getenv("REDIS_HOST", "redis")
QUEUE_NAME = "sil_jobs"

r = redis.Redis(host=REDIS_HOST, decode_responses=True)


def process_job(job: dict) -> None:
    result = run_sil_pipeline(job["input"])
    r.set(f"sil_result:{job['id']}", json.dumps(result))
    print(f"Processed job {job['id']}: {result['decision']}")


def main() -> None:
    print(f"Worker listening on Redis queue '{QUEUE_NAME}'...")
    while True:
        try:
            _, job_data = r.blpop(QUEUE_NAME, timeout=0)
            job = json.loads(job_data)
            process_job(job)
        except Exception as e:
            print(f"Error: {e}")
            time.sleep(1)


if __name__ == "__main__":
    main()
