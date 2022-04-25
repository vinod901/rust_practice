
provider "aws" {                     
    region = "ap-south-1"            
}

variable "vpc-cidr_block" {}
variable "subnet1-cidr_block" {}
variable "avail_zone" {}
variable "env_prefix" {}
variable "my_ip" {} 
variable "instance_type" {}
variable "public_key_location" {}
  
resource "aws_vpc" "myapp-vpc" {    
    cidr_block = var.vpc-cidr_block            
    tags = {
        Name: "${var.env_prefix}-vpc"
    } 
}



resource "aws_subnet" "myapp-subnet-1" {     
   vpc_id = aws_vpc.myapp-vpc.id
   cidr_block = var.subnet1-cidr_block             
    availability_zone = "ap-south-1a"      
    tags = {
      Name: "${var.env_prefix}-subnet-1"
    }
}


resource "aws_internet_gateway" "myapp-igw" {
  vpc_id = aws_vpc.myapp-vpc.id
  tags = {
    Name: "${var.env_prefix}-igw"
  }
  
}

// Creating  route table 

resource "aws_route_table" "mypp-route-table" {
  vpc_id = aws_vpc.myapp-vpc.id
  route {
    cidr_block = "0.0.0.0/0"
    gateway_id = aws_internet_gateway.myapp-igw.id
  }
  tags = {
    Name: "${var.env_prefix}-rtb"
  }
}
# Creating  subnet Association
resource "aws_route_table_association" "a-rtb-subnet" {
  subnet_id = aws_subnet.dev-subnet-1.id
  route_table_id = aws_route_table.mypp-route-table.id
}

resource "aws_default_route_table" "main-rtb" {
  default_route_table_id = aws_vpc.myapp-vpc.default_route_table_id

  route {
     cidr_block = "0.0.0.0/0"
    gateway_id = aws_internet_gateway.myapp-igw.id
  }
  tags = {
    Name: "${var.env_prefix}-main-rtb"
  }
  
}

# Creating security group -- configuring incoming & outgoing ports
resource "aws_security_group" "myapp-sg" {
  name = "myapp-sg"
  vpc_id = aws_vpc.myapp-vpc.id
#incoming ports need to be opend
  ingress {
    from_port = 22
    to_port = 22
    protocol = "tcp"
    cidr_blocks = [var.my_ip]
  }
  ingress {
    from_port = 8080
    to_port = 8080
    protocol = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }
  # Outgoing ports need to be opend
  egress {
    from_port = 0
    to_port = 0
    protocol = "-1"
    cidr_blocks = ["0.0.0.0/0"]
    prefix_list_ids = []
  }
  tags = {
    Name: "${var.env_prefix}-sg"
  }
  
}  


resource "aws_default_security_group" "default-sg" {
  vpc_id = aws_vpc.myapp-vpc.id
  ingress {
    from_port = 22
    to_port = 22
    protocol = "tcp"
    cidr_blocks = [var.my_ip]
  }
  ingress {
    from_port = 8080
    to_port = 8080
    protocol = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }
  egress {
    from_port = 0
    to_port = 0
    protocol = "-1"
    cidr_blocks = ["0.0.0.0/0"]
    prefix_list_ids = []
  }
  tags = {
    Name: "${var.env_prefix}-default-sg"
  }
}

data "aws_ami" "latest-amazon-linux-image" {
  most_recent = true
  owners = ["amazon"]
  filter {
    name = "name"
    values = ["amzn2-ami-hvm-*-x86_64-gp2"]

  }
  filter {
    name = "virtualization-type"
    values = ["hvm"]
  }
}

output "aws_ami_id" {
  value = data.aws_ami.latest-amazon-linux-image.id
}

output "ec2_public_ip" {
  value = aws_instance.myapp-server.public_ip
}

resource "aws_key_pair" "ssh-key" {
  key_name = "server-key"
  public_key = file(var.public_key_location)
  
}

resource "aws_instance" "myapp-server" {
  ami = data.aws_ami.latest-amazon-linux-image.id
  instance_type = var.instance_type
  subnet_id = aws_subnet.myapp-subnet-1.id
  vpc_security_group_ids = [aws_default_security_group.default-sg.id]
  availability_zone = var.avail_zone
  associate_public_ip_address = true    
  key_name = aws_key_pair.ssh-key.key_name
  user_data = file("entry-script.sh")            
  tags = {
    Name = "${var.env_prefix}-server"
    //foo = "bar"
  }
}

terraform {
  required_version = ">= 1.0.4"
  required_providers {
    postgresql = {
      source  = "cyrilgdn/postgresql"
      version = ">= 1.15.0"
    }
  }
}
//------------------------------------------------------------------------------------------
provider "postgresql" {
  scheme    = "awspostgres"
  host      = "db.domain.name"
  port      = "5432"
  username  = "vinod"
  password  = "gokul"
  superuser = false
}
resource "postgresql_role" "new_db_role" {
    name                = "admin"
    login               = true
    password            = "db_password"
    encrypted_password  = true
}
resource "postgresql_database" "new_db" {
  name              = "vinod_db"
  owner             = "vinod"
  template          = "template0"
  lc_collate        = "C"
  connection_limit  = -1
  allow_connections = true
}
//------------------------------------------------------------------------------------------------------------

provider "postgresql" {
  alias            = "pgadm"
  host             = var.dbhost
  port             = var.dbport
  username         = var.pgadmin_user
  sslmode          = var.sslmode
  connect_timeout  = var.connect_timeout
  superuser        = var.superuser
  expected_version = var.expected_version
}
provider "postgresql" {
  alias            = "pgmgm"
  host             = var.dbhost
  port             = var.dbport
  database         = var.inputs["db_name"]
  username         = var.pgadmin_user
  sslmode          = var.sslmode
  connect_timeout  = var.connect_timeout
  superuser        = var.superuser
  expected_version = var.expected_version
}

// some dummy line