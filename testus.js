console.log("hallo");
/*
href="https://github.com/wireapp/don-bot     
 neue zeile  noch ne Zeile  
 
 weitere Zeile
Hi, I'm GitHub-Bot. Here is how to set me up:

1. Go to the repository that you want to connect to
2. Go to Settings / Webhooks / Add webhook
3. Add Payload URL: https://35.187.99.125/github/e7776d0a-55f4-4997-8816-651d61137130
4. Set Content-Type: application/json
5. Disable SSL verification
6. Set Secret: hl17ak
Text   einfügen


Hi, I'm GitHub-Bot. Here is how to set me up:

1. Go to the repository that you want to connect to
2. Go to Settings / Webhooks / Add webhook
3. Add Payload URL: https://35.187.99.125/github/e7776d0a-55f4-4997-8816-651d61137130
4. Set Content-Type: application/json
5. Disable SSL verification
6. Set Secret: hl17ak


devbot
 testus1

Usage:
register
get self
create bot
list my bots
show bot <name>
update bot <name>
delete bot <name>
enable bot <name>
test bot <name>
create public channel

get self
Dev
{
  "name" : "testus1 Robert Pfafferodt",
  "email" : "wire2@robisys.de",
  "url" : "https:",
  "description" : "You know, for the bots",
  "id" : "e1791f6c-ded5-461d-978d-cebd0d52b65e"
}


 Dev
What is the base url for this bot?

testus1
http://wirebot.robisys.de

Dev
Please, specify valid https url like: https://example.com

testus1
https://wirebot.robisys.de

Dev
Write some description for this bot

testus1
robit bot test

Dev
Paste the URL for the profile picture

testus1
https://wirebot.robisys.de/profilpicture.jpg

Dev

Paste rsa public key here


----------
https://stackoverflow.com/questions/34598363/get-public-key-from-ssh-server

Download public key from github and save it into github.pub file:

ssh-keyscan -t rsa github.com | sed "s/^[^ ]* //" > github.pub

Convert SSH public key format into X.509 public key format:

ssh-keygen -f github.pub -e -m pem > github.pem

Parse ASN.1 encoding of key to obtain public key modulus and exponent:

cat github.pem | sed "/--/d" | openssl asn1parse | grep "INTEGER" | sed "s/.*://"


*/
