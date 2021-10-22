if [ -n "$1" ] 
then
    sed -i "s|{{base_url}}|$1|g" client/build/config.js
fi

./server/secret-share