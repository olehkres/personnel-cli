# personnel-cli
Personnel-cli is one of Personnel Manager interfaces.

You may also need personnel-api if you want to run it locally.

#Usage
Download the package for your OS and start to fill up db.sqlite with your personnel staff list.
Note: if you are not a Ukrainian, you probably want to edit personnel-api, so it can fit in your staff list.

After filling up your database just use it like this:
```
personnel-cli 0 ./templates/holiday.html ./templates/reference.html ./db.sqlite
```

Fell free to edit and add new templates. It is just html. You can get variables list from database personnel view. 
