required_arg() {
    if [ -z "$1" ]; then
        echo "Required argument $2 missing"
        exit 1
    fi
}
