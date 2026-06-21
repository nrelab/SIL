import json
import os
import time

import redis

from sil_queue import SilQueue

REDIS_HOST = os.getenv("REDIS_HOST", "redis")


def process_job(job: dict) -> dict:
    text = job.get("input", "")

    normalized = text
    flags = []
    decision = "ALLOW"

    if "\u0192" in text:
        flags.append("CONFUSABLE_DETECTED")
        normalized = text.replace("\u0192", "f")
        decision = "BLOCK"

    if "\u200b" in text:
        flags.append("ZERO_WIDTH_DETECTED")
        decision = "BLOCK"

    return {
        "input": text,
        "normalized": normalized,
        "flags": flags,
        "decision": decision,
    }


def main() -> None:
    q = SilQueue(host=REDIS_HOST)
    print(f"Worker listening on queue '{q.queue_name}'...")

    while True:
        try:
            job = q.dequeue()
            if job:
                result = process_job(job)
                q.complete(job["id"], result)
                print(f"Processed job {job['id']}: {result['decision']}")
            time.sleep(0.1)
        except Exception as e:
            print(f"Error: {e}")
            time.sleep(1)


if __name__ == "__main__":
    main()
