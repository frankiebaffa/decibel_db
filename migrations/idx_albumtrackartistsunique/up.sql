create unique index BangersDb.AlbumTrackArtistsUnique on AlbumTrackArtists (Artist_Id, AlbumTrack_Id, ArtistType_Id) where Active = 1;