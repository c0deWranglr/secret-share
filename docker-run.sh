if [ -n "$1" ] 
then
    sed -i "s|{{base_url}}|$1|g" client/build/config.js
fi

if [ -n "$2" ]
then
    sed -i "s|{{site_name}}|$2|g" client/build/config.js
fi

./server/secret-share