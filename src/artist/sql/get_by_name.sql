select *
from DecibelDb.Artist as artist
where artist.Name = :name
and artist.Active = 1;
