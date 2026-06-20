provider "aws" {
  region = var.aws_region
}

resource "aws_ecs_cluster" "sil" {
  name = "sil"
}

resource "aws_ecs_service" "api" {
  name            = "sil-api"
  cluster         = aws_ecs_cluster.sil.id
  task_definition = aws_ecs_task_definition.api.arn
  launch_type     = "FARGATE"
  desired_count   = 3

  network_configuration {
    subnets         = var.subnet_ids
    security_groups = [aws_security_group.sil.id]
  }
}

resource "aws_rds_instance" "postgres" {
  engine         = "postgres"
  engine_version = "15"
  instance_class = "db.t3.medium"
  db_name        = "sil"
  username       = "sil"
  password       = var.db_password
  skip_final_snapshot = true
}

resource "aws_elasticache_cluster" "redis" {
  cluster_id  = "sil-redis"
  engine      = "redis"
  node_type   = "cache.t3.micro"
  num_cache_nodes = 1
}

resource "aws_security_group" "sil" {
  name = "sil-sg"

  ingress {
    from_port   = 443
    to_port     = 443
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }
}

variable "aws_region" {
  default = "us-east-1"
}

variable "subnet_ids" {
  type = list(string)
}

variable "db_password" {
  type      = string
  sensitive = true
}

output "api_endpoint" {
  value = aws_ecs_service.api.id
}
