select album.*
from DecibelDb.Albums as album
where album.AlbumType_Id = :albumtype_id;
