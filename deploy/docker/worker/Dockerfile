FROM python:3.11-slim

WORKDIR /app

COPY deploy/docker/worker/requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt

COPY api/sil_api/core_bridge.py /app/
COPY deploy/docker/worker/worker.py /app/

CMD ["python", "worker.py"]
