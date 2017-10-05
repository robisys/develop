
ssh-keyscan -t rsa wirebot.robisys.de | sed "s/^[^ ]* //"  > wirebot1.pub 

#ssh-keyscan -t rsa wirebot.robisys.de 
#   > wirebot.pub 

ssh-keygen -f wirebot1.pub -e -m pem  >wirebot1.pem 
