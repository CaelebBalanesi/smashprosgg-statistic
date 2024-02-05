import { Component } from '@angular/core';

export interface Character {
  name: string,
  img: string,
}

@Component({
  selector: 'app-character-selecter',
  standalone: true,
  imports: [],
  templateUrl: './character-selecter.component.html',
  styleUrl: './character-selecter.component.scss'
})
export class CharacterSelecterComponent {

  characters: Character[] = [
    {
      name: "Ness",
      img: "https://storage.googleapis.com/smashpros.gg/characters/icons/ness.png",
    },
    {
      name: "Aegis",
      img: "https://storage.googleapis.com/smashpros.gg/characters/icons/homura.png"
    }, {
      name: "Mario",
      img: "https://storage.googleapis.com/smashpros.gg/characters/icons/mario.png"
    }
  ]
}
