# SchackbibLIotekAM (årets bästa kod fosho)
## Allmän Info
### main.rs
main.rs finns så att du kan se en implementation av biblioteket och hur man använder de flesta public functions och metoderna.
### boardinfo
Ett format för en position som jag kör på:\
De första 2 karaktärerna är positionen (på "a1"-format) där en-passant-rutan är, ifall brädet inte har en en-passant-ruta blir detta istället "XX" \
Sedan följer 1 karaktär; vems tur det är. Antingen "W" för vit, eller "B" för svart \
Sedan följer 64 karaktärer; hela brädet på en linje, den printar ut allting som finns i "board" - som skrivet nedanför, det första som skrivs ut är ruta a1 följt av b1... .\
Därefter följer ett '+'-tecken och ett tal; hur många halvdrag som hade gjorts när senaste capturen/pawn advance hände.\
Sedan följer ett till '+'-tecken och ett till tal; hur många halvdrag som har gjorts hittils.
### Användning av FEN
FEN används inte i implementationen av schackspelet, däremot finns det funktioner för att ta in FEN-strängar/listor för att göra om till boardinfo och sedan användas.\
Funktionen load_from_fen() finns för att fylla detta syfte.
### Dragformatering
Alla drag formateras som "e1d2", alltså från vilken position du vill göra draget följt av positionen den pjäsen ska flyttas till. \
Rockad görs antingen som en kungförflyttning (exempelvis rockad för vit på damsidan blir "e1c1"), eller genom "O-O"/"O-O-O" för att signalera kungsiderockad/damsiderockad för den nuvarande spelaren. I funktionen "print_all_moves" visas dessa endast på formatet "e1c1" men man kan skriva in "O-O" och den förstår. \
Promotion fungerar genom att du skriver det vanliga draget (exempelvis "a7a8", bonden som står på a7 ska flyttas till a8), följt av den pjäs du vill promote:a till med stor bokstav (dam - 'Q', springare - 'N', löpare - 'B', torn - 'R').
Exempelvis a7a8Q för att promote:a till en dam på ruta a8. 
## Board
Brädet innehåller 7 saker:
### board
Ett 64 stort fält med chars, varje char är vilken pjäs som befinner sig på en plats. \
Conversion från koordinat (x, y) till position är 8*x+y\
Liten bokstav betyder att pjäsen är vit och stor bokstav betyder att pjäsen är svart (ex 'p' är vit bonde medan 'P' är svart bonde)\
'p' - bonde, 'r' - torn, 'b' - löpare, 'n' - springare, 'k' - kung, 'q' - dam
### start
En i64 som kan anamma 2 värden: \
0: Då är det svarts tur \
1: Då är det vits tur
### history
En vector av strings som innehåller information om brädet efter ett givet drag.\
De följer formatet boardinfo som jag definierade tidigare.
### castle
Ett 4 stort fält med bools, varje bool är ifall den typen av rockad funkar:\
Om castle[0] är true kan vit göra rockad på kungsida \
Om castle[1] är true kan vit göra rockad på damsidan \
Om castle[2] är true kan svart göra rockad på kungsidan \
Om castle[3] är true kan svart göra rockad på damsidan
### en_passant_square
En i64 som ger positionen som en bonde kan gå till ifall den gör en passant\
Exempelvis om vi säger att vit gör draget d2->d4 (med detta format d2d4) blir brädets en passant värde det numeriska värdet för d3, vilket hade blivit 19.
### last_capture
En i64 som säger vilket halvdrag som senast var en capture.\
Om en pjäs slår en annan pjäs blir denna uppdaterad till det nuvarande draget. 
### time_now
En i64 som säger vad tiden är nu, räknat i antalet halvdrag som gjorts.\
Till exempel är tiden efter 10 fulla drag (vit har gjort 10 drag, svart har gjort 10 drag) 20. 
## Viktiga funktioner
### filtered_moves (public funktion)
filtered_moves tar in ett mutable bräde och returnerar alla lagliga drag på formatet "d2d4" i den nuvarande positionen för den nuvarande spelaren
### get_amount_moves (public funktion)
get_amount_moves tar också in ett mutable bräda fast returnerar antalet lagliga drag i form av en i64.
### is_over (public funktion)
is_over tar in ett mutable bräde och returnerar ett av 5 värden:\
0 -> partiet är inte över \
1 -> schackmatt \
2 -> patt \
3 -> 3 fold repetition \
4 -> 50 move rule 
### get_start (public funktion)
get_start tar in ett mutable bräde och returnerar en i64; vems tur det är (0 -> svart, 1 -> vit)
### load_from_fen (public funktion)
load_from_fen tar in ett mutable bräde samt en Vec<String> som innehåller delarna av en FEN, och loadar den till brädet.
### fenstring_to_vec (public funktion)
fenstring_to_vec tar in en String och returnerar en Vec<String>, den splittrar upp delarna av en fen string och returnerar det så som load_from_fen vill ha det.
### print_history (public funktion)
print_history tar in ett mutable bräde och printar brädets history utan att returnera någonting
### load_from_info (public funktion)
load_from_info tar in ett mutable bräde samt en String (brädets boardinfo) och gör så att brädet får den infon. 
## Viktiga metoder
### board.init_board()
board.init_board() tar inte in någonting och sätter brädet till schacks startposition
### board.print_board()
board.print_board() tar inte in någonting och printar brädet i 2d
### board.get_board()
board.get_board() tar inte in någonting och returnerar brädets boardinfo
### board.switch_start()
board.switch_start() tar inte in någonting och ändrar vilken spelare som startar
