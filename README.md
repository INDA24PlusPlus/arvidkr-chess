# Tjackbibliotek (jätteful kod 😭)
## Allmän Info
### boardinfo
Ett format för en position som jag kör på:\
De första 2 karaktärerna är positionen (på "a1"-format) där en-passant-rutan är, ifall brädet inte har en en-passant-ruta blir detta istället "XX" \
Sedan följer 1 karaktär; vems tur det är. Antingen "W" för vit, eller "B" för svart \
Sedan följer 64 karaktärer; hela brädet på en linje, den printar ut allting som finns i "board" - som skrivet nedanför.
## Board
Brädet innehåller 6 saker:
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
En i64 som säger vilket drag som senast var en capture.\
Om en pjäs slår en annan pjäs blir denna uppdaterad till det nuvarande värdet.
## Viktiga funktioner
### 
