import { Component, EventEmitter, Input, Output } from '@angular/core';
import { Character } from '../../pages/stats-page/stats-page.component';


@Component({
  selector: 'app-character-selecter',
  standalone: true,
  imports: [],
  templateUrl: './character-selecter.component.html',
  styleUrl: './character-selecter.component.scss'
})
export class CharacterSelecterComponent {
  @Input("characters") characters!: Character[];
  @Output("selectedCharacter") selectedCharacter = new EventEmitter<string>();

  selectCharacter(characterName: string){
    this.selectedCharacter.emit(characterName);
  }
}
