import ephem

# Define as datas iniciais e finais
start_date = ephem.Date('2023/06/05')
end_date = ephem.Date('2023/06/06')

# Cria os objetos para os cinco primeiros planetas
mercury = ephem.Mercury()
venus = ephem.Venus()
mars = ephem.Mars()
jupiter = ephem.Jupiter()
saturn = ephem.Saturn()

# Loop para calcular as posições dos planetas em intervalos de um dia
current_date = start_date
while current_date < end_date:
    # Calcula as posições dos planetas para a data atual
    mercury.compute(current_date)
    venus.compute(current_date)
    mars.compute(current_date)
    jupiter.compute(current_date)
    saturn.compute(current_date)

    # Imprime as posições dos planetas para a data atual
    print("Data:", current_date)
    print("Mercúrio:", mercury.ra, mercury.dec)
    print("Vênus:", venus.ra, venus.dec)
    print("Marte:", mars.ra, mars.dec)
    print("Júpiter:", jupiter.ra, jupiter.dec)
    print("Saturno:", saturn.ra, saturn.dec)
    print("")

    # Incrementa a data em um dia
    current_date += 1
