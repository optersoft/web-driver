#!/bin/bash

#google-chrome --version

# chromium-browser -version
# https://sites.google.com/chromium.org/driver/ (check latest)

VERSION=100.0.4896.60
FILE=${HOME}/.local/bin/chromedriver

wget https://chromedriver.storage.googleapis.com/${VERSION}/chromedriver_linux64.zip
unzip chromedriver_linux64.zip
rm chromedriver_linux64.zip

rm -f ${FILE}
mv chromedriver ${FILE}
#chown root:root /usr/bin/chromedriver
chmod +x ${FILE}