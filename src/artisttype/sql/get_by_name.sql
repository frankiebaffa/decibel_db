select artype.*
from BangersDb.ArtistType as artype
where artype.Name = :name
ane artype.Active = 1;
