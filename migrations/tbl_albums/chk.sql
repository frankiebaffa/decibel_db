select count(name)
from DecibelDb.sqlite_master
where type = 'table'
and name = 'Albums';
