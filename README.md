
# Disk Watcher

`Disk Watcher` je jednoduchá aplikace v jazyce Rust, která sleduje složky v souborovém systému, vyhodnocuje jejich velikost a ukládá výsledky do SQLite databáze.

## Použití

### Argumenty příkazové řádky

Aplikace přijímá následující argumenty:

- `--watcher-path`: Cesta ke složce, kterou chcete monitorovat. (Výchozí: `/`)
- `--watcher-depth`: Maximální hloubka adresářové struktury, která bude prohledávána. (Výchozí: `2`)
- `--watcher-same-fs`: Sledujte pouze stejné souborové systémy jako počáteční cesta. (Výchozí: `true`)
- `--watcher-count`: Počet největších adresářů, které budou zobrazeny. (Výchozí: `10`)
- `--db-path`: Cesta k SQLite databázi, do které budou ukládány výsledky. (Výchozí: `/data/data.sqlite`)

Příklad spuštění:

```bash
./disk-watcher --watcher-path /var/log --watcher-depth 3 --watcher-count 5 --db-path /var/db/data.sqlite
```

### Environmentální proměnné

Každý z výše uvedených argumentů lze nastavit také prostřednictvím environmentálních proměnných:

- `WATCHER_PATH`: Cesta ke složce (ekvivalent `--watcher-path`).
- `WATCHER_DEPTH`: Hloubka prohledávání složek (ekvivalent `--watcher-depth`).
- `WATCHER_SAME_FS`: Sledujte pouze stejné souborové systémy (ekvivalent `--watcher-same-fs`).
- `WATCHER_COUNT`: Počet největších adresářů (ekvivalent `--watcher-count`).
- `DB_PATH`: Cesta k SQLite databázi (ekvivalent `--db-path`).

Příklad použití s environmentálními proměnnými:

```bash
export WATCHER_PATH="/home/user/documents"
export WATCHER_DEPTH="3"
export WATCHER_COUNT="5"
export DB_PATH="/home/user/data.sqlite"
./disk-watcher
```

## Logování

Aplikace používá `env_logger` pro logování. Pro konfiguraci logování můžete nastavit environmentální proměnnou `RUST_LOG`.

Příklad nastavení úrovně logování na `info`:

```bash
export RUST_LOG=info
./disk-watcher
```

## Build a použití Docker/Podman
Aplikace používá multi-stage build pro vytvoření lehkého image s využitím Debianu a Rustu. Build proces zahrnuje sestavení binárního souboru v Rustu a jeho následné nasazení do lehkého Debian image.

## Použití Makefile
Projekt obsahuje Makefile, který usnadňuje build a push Docker/Podman image:

- `make images`: Vytvoří image pro aplikaci.
- `make push`: Nahraje image na zadaný registr.
- `make magic`: Spustí aplikaci uvnitř kontejneru.

### Příklad spuštění kontejneru
Pro spuštění aplikace v kontejneru s připojeným volume, například:
```bash
podman run -it -v ./test:/data:Z --rm localhost/disk-watcher
```

Tento příkaz spustí aplikaci, která bude monitorovat složky dle nastavených argumentů nebo proměnných a ukládat výsledky do SQLite databáze umístěné v /data (na hostu `./test`)
