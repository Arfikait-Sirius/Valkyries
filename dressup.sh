#!/bin/sh

DIR_VALKYRIES="$1/valkyries"
if [ ! -d $DIR_VALKYRIES ]; then
     mkdir $DIR_VALKYRIES
fi

cp ./dir_Girls/**/*.rs $DIR_VALKYRIES/.

cp ./dir_Modules/valkyries.rs $1/.

echo Completed!
