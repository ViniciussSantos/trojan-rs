
echo "[>- Update and upgrade]"
sudo apt update >/dev/null 2>&1
sudo apt install -y curl >/dev/null 2>&1
echo "[>- Install and configure Docker]"
sudo apt install -y docker.io >/dev/null 2>&1
sudo systemctl enable docker >/dev/null 2>&1
sudo cat <<EOF | sudo tee /etc/docker/daemon.json
{ "exec-opts": ["native.cgroupdriver=systemd"],
"log-driver": "json-file",
"log-opts":
{ "max-size": "100m" },
"storage-driver": "overlay2"
}
EOF
sudo systemctl restart docker >/dev/null 2>&1
echo "[>- create postgres container]"
sudo docker run -d \
  --name server-db \
  -e POSTGRES_DB=trojan \
  -e POSTGRES_USER=postgres \
  -e POSTGRES_PASSWORD=123 \
  -p 5432:5432 \
  postgres:latest
