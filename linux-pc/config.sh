# config.sh
FILE=config.lst
echo "config"  > $FILE
date >> $FILE
#uname -a  >>$FILE
echo "os-release"  >>$FILE
cat /etc/os-release   >>$FILE
