## **Requirements**
- Docker
- Docker-compose
- Rust


## **Inside the directory**

### **Step 1**
Up Sql Server Docker

```sh
docker-compose up --build
```

#### **Step 2**
Connect in SqlServer 

#### **Step 3**
Run the sql file "TempTest.sql" on SQL Server to create and insert data.

#### **Step 4**
Run Rust Code

```sh
cargo run
```