
#  Create service key and certificate:

openssl genrsa -out server.key 4096
openssl req -new -key server.key -out csr.pem
openssl x509 -req -days 7300 -in csr.pem -signkey server.key -out server.crt
openssl rsa -in server.key -pubout -out pubkey.pem






ssh-keyscan -t rsa wirebot.robisys.de | sed "s/^[^ ]* //"  > wirebot1.pub 

#ssh-keyscan -t rsa wirebot.robisys.de 
#   > wirebot.pub 

ssh-keygen -f wirebot1.pub -e -m pem  >wirebot1.pem 
