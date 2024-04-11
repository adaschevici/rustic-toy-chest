#!/bin/bash
# Generate a JWT secret
# bash generate-JWT-secret.sh
# Output: JWT secret

echo "Choose an option:"

ORIGINAL_COLUMNS=$COLUMNS
# Set COLUMNS to a small number to try and force wrapping
COLUMNS=1

options=("Generate using openssl" "Generate using urandom" "Generate using node" "Quit")

select opt in "${options[@]}"
do
    case $opt in
        "Generate using openssl")
            openssl rand -hex 32
            echo "You chose to generate using openssl"
            ;;
        "Generate using urandom")
            head -c 32 /dev/urandom | xxd -p -c 64
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

COLUMNS=$ORIGINAL_COLUMNS
