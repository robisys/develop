#   
ssh-keyscan -t rsa wirebot.robisys.de | sed "s/^[^ ]* //"  > wirebot1.pub 
ssh-keygen -f wirebot1.pub -e -m pem >wirebot1.pem 
