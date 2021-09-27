select *
from BangersDb.Artist as artist
where artist.Name = :name
and artist.Active = 1;
