### Docker Stuff

```
# Start the database
docker run --rm -p 5500:5500 -e "POSTGRES_PASSWORD=postgres" --name pg postgres:14

# optional psql (other terminal)
docker exec -it -u postgres pg psql
```

### Dev Test

- `-q` quiet
- `-c` clear
- `-w src/` watch the src folder
- `-x` execute `test` on `model_db_` (`test model::db::tests`) and accepts to more flags `--` one for unique thread `--test-threads=1` and other `--nocapture` which allows **println in development mode**

```
# Test for model
cargo watch -q -c -w src/ -x 'test model_ -- --test-threads=1 --nocapture'
```
