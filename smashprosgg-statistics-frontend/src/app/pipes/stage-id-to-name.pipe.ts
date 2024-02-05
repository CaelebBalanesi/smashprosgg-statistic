import { Pipe, PipeTransform } from '@angular/core';

@Pipe({
  name: 'stageIdToName',
  standalone: true
})
export class StageIdToNamePipe implements PipeTransform {
  conversion: { [key: string]: string } = {
    "12": "Pokemon Stadium",
    "13": "Battlefield",
    "14": "Small Battlefield",
    "15": "Smashville",
    "16": "Town and City",
    "7": "Final Destination",
    "17": "Hollow Bastion",
    "18": "Kalos Pokemon League"
  }

  transform(value: string, ...args: unknown[]): string | undefined {
    console.log(value);
    console.log(this.conversion[value]);
    return this.conversion[value];
  }

}
