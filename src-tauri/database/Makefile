FLYWAY_VERSION = 11.10.4
DUCKDB_JDBC_VERSION = 1.1.3

## Install flyway for managing database scripts
flyway-symlink:
	sudo ln -sf /opt/flyway/flyway /usr/local/bin/flyway

flyway-duckdb-driver:
	wget https://repo1.maven.org/maven2/org/duckdb/duckdb_jdbc/$(DUCKDB_JDBC_VERSION)/duckdb_jdbc-$(DUCKDB_JDBC_VERSION).jar
	mkdir -p /opt/flyway/drivers
	mv duckdb_jdbc-$(DUCKDB_JDBC_VERSION).jar /opt/flyway/drivers/

flyway-cli-install:
	wget https://github.com/flyway/flyway/releases/download/flyway-$(FLYWAY_VERSION)/flyway-commandline-$(FLYWAY_VERSION)-linux-x64.tar.gz
	tar -xzf flyway-commandline-$(FLYWAY_VERSION)-linux-x64.tar.gz
	sudo mv flyway-$(FLYWAY_VERSION) /opt/flyway
	sudo rm flyway-commandline-$(FLYWAY_VERSION)-linux-x64.tar.gz

flyway-install:
	$(MAKE) flyway-cli-install
	$(MAKE) flyway-symlink
	$(MAKE) flyway-duckdb-driver

flyway-auth:
	flyway auth -IAgreeToTheEula

flyway-uninstall:
	sudo rm -rf /opt/flyway
	sudo rm -f /usr/local/bin/flyway

migrate:
	flyway migrate

repair:
	flyway repair

flyway-all: flyway-install flyway-symlink flyway-duckdb-driver
