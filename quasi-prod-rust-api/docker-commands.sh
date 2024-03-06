docker image ls --format '{{.CreatedAt}}\t{{.ID}}\t{{.Repository}}:{{.Tag}}' | awk -v date="$(date +%Y-%m-%d)" '$1 ~ date {print $(NF-1),$NF}'
