# some orders
cargo init cr8s

## docker
docker-compose up -d build
docker-compose up -d
docker-compose logs postgres
docker-compose ps

## docker diesel
docker-compose exec app diesel setup
docker-compose up -d
docker-compose exec app diesel migration generate create_rustaceans
docker-compose exec app diesel migration generate create_crates

// add database to those .sql files

docker-compose exec app diesel migration run