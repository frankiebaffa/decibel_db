select count(name)
from BangersDb.sqlite_master
where type = 'table'
and name = 'Songs';
