f() {
    host=$(git config deploy.host)
    dir=$(git config deploy.dir)
    source "deploy.sh"

    echo "$host --> $dir"

    if [[ $host = '' ]] || [[ $dir = '' ]]
    then
        echo 'Git config deploy is not configured.'
        return
    fi

    IFS=','
    for h in $host
    do
        echo "- $h"
    done
    return

    git pp
    echo '\033[2m'
    ssh -A $host 'cd '$dir' && git ff && git log -1'
    echo '\033[0m'
}

f

echo $IFS
