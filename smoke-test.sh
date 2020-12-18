printf 'curl -i -X GET localhost:3000/cache/foo'
printf '\nexpect a 204 (if starting from empty)\n'
curl -i -s -X GET localhost:3000/cache/foo | head -n 1

printf "\ncurl -i -X POST localhost:3000/cache/foo -d 'hello world!'"
printf '\nexpect a 200\n'
curl -i -s -X POST localhost:3000/cache/foo -d 'hello world!' | head -n 1

printf '\ncurl -i -X GET localhost:3000/cache/foo'
printf '\nexpect a 200 and "hello world!"\n'
curl -i -s -X GET localhost:3000/cache/foo | grep -v 'Content-' | grep -v 'Date:'

printf "\ncurl -i -X PUT localhost:3000/cache/foo -d 'how are you?'"
printf '\nexpect a 200\n'
curl -i -s -X PUT localhost:3000/cache/foo -d 'how are you?' | head -n 1

printf '\ncurl -i -X GET localhost:3000/cache/foo'
printf '\nexpect a 200 and "how are you?"\n'
curl -i -s -X GET localhost:3000/cache/foo | grep -v 'Content-' | grep -v 'Date:'

printf '\ncurl -i -X DELETE localhost:3000/cache/foo'
printf '\nexpect a 200\n'
curl -i -s -X DELETE localhost:3000/cache/foo | head -n 1

printf '\ncurl -i -X GET localhost:3000/cache/foo'
printf '\nexpect a 204\n'
curl -i -s -X GET localhost:3000/cache/foo | head -n 1

