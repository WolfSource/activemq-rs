#! /bin/sh

#we need following packages
echo ++++++++++++++++++++++++++++++++++++++++++
echo getting necessary packages
echo ++++++++++++++++++++++++++++++++++++++++++

if [ -f /etc/lsb-release ]; then
  sudo apt-get install -y git autoconf libtool libapr1 libapr1-dev libssl-dev
elif [ -f /etc/redhat-release ]; then
  sudo yum install -y git autoconf libtool libapr1 libapr1-dev libssl-dev
fi

#grab the activemq-cpp source code
echo ++++++++++++++++++++++++++++++++++++++++++
echo cloning activemq-cpp
echo ++++++++++++++++++++++++++++++++++++++++++
git clone https://git-wip-us.apache.org/repos/asf/activemq-cpp.git

# go to active-mq folder
echo ++++++++++++++++++++++++++++++++++++++++++
echo building activemq
echo ++++++++++++++++++++++++++++++++++++++++++
cd ./activemq-cpp/activemq-cpp/
sudo chmod +x ./autogen.sh
./autogen.sh
./configure
make
sudo make install
