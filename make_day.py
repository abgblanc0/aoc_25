import os
import sys

TEMPLATE_MOD = """mod part1;
mod part2;

use std::fs;

pub fn solution() {{
  let input: String = fs::read_to_string("src/days/day{day}/input.txt").unwrap();
  part1::solution(&input);
  part2::solution(&input);
}}
"""

TEMPLATE_PART = """pub fn solution(_input: &str) {{
  println!("Day {day} - Part X not implemented yet");
}}
"""


def main():
    if len(sys.argv) != 2:
        print("Uso: python make_day.py <dia>")
        return

    day = sys.argv[1]
    folder = f"src/days/day{day}"

    os.makedirs(folder, exist_ok=True)

    # Crear input.txt vacío
    open(os.path.join(folder, "input.txt"), "w").close()

    # Crear mod.rs dentro del dayXX
    with open(os.path.join(folder, "mod.rs"), "w") as f:
        f.write(TEMPLATE_MOD.format(day=day))

    # Crear part1.rs y part2.rs
    for part in ["part1", "part2"]:
        with open(os.path.join(folder, f"{part}.rs"), "w") as f:
            f.write(TEMPLATE_PART.format(day=day))

    print(f"✓ Día {day} generado en {folder}")

    # --- Actualizar src/days/mod.rs ---
    days_mod_path = "src/days/mod.rs"
    new_line = f"\npub mod day{day};"

    # Si mod.rs no existe, crearlo
    if not os.path.exists(days_mod_path):
        with open(days_mod_path, "w") as f:
            f.write(new_line)
        print(f"✓ Creado {days_mod_path} con {new_line.strip()}")
        return

    # Leer mod.rs
    with open(days_mod_path, "r") as f:
        content = f.readlines()

    # No añadir si ya existe
    if new_line in content:
        print(f"✓ mod.rs ya contiene 'pub mod day{day};'")
        return

    # Añadir línea al final
    with open(days_mod_path, "a") as f:
        f.write(new_line)

    print(f"✓ Añadido 'pub mod day{day};' a src/days/mod.rs")


if __name__ == "__main__":
    main()
