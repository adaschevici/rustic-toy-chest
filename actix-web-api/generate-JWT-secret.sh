#!/bin/bash
# Generate a JWT secret
# bash generate-JWT-secret.sh
# Output: JWT secret
echo "Choose an option:"
options=("Generate using openssl" "Generate using urandom" "Generate using node" "Quit")

select opt in "${options[@]}"
do
    case $opt in
        "Generate using openssl")
            openssl rand -base64 32
            echo "You chose to generate using openssl"
            ;;
        "Generate using urandom")
            head /dev/urandom | tr -dc A-Za-z0-9 | head -c 32 ; echo ''
            echo "You chose to generate using urandom"
            ;;
        "Generate using node")
            echo "You chose to generate using node"
            node -e "console.log(require('crypto').randomBytes(32).toString('hex'))"
            ;;
        "Quit")
            break
            ;;
        *) echo "invalid option $REPLY";;
    esac
done
