import { Component } from '@angular/core';
import { BackendService } from '../../services/backend.service';
import { NgxEchartsDirective, provideEcharts } from 'ngx-echarts';
import { EChartsOption, LineSeriesOption, BarSeriesOption } from 'echarts';

export interface DateData {
  wins: number,
  total: number,
}

@Component({
  selector: 'app-stats-page',
  standalone: true,
  imports: [ NgxEchartsDirective ],
  templateUrl: './stats-page.component.html',
  styleUrl: './stats-page.component.scss',
  providers: [
    provideEcharts(),
  ]
})
export class StatsPageComponent {

  options: EChartsOption | undefined;

  constructor(private backend: BackendService) {}

  data: string = "";
  flag: boolean = true;
  game_winrate: number = 0;
  loading: boolean = true;

  get_winrate() {
    this.backend.get_set_winrate(this.backend.username).subscribe(data => {
      console.log(data);
      this.game_winrate = data;
      this.flag = false;
    });
  }
  get_map_winrate() {
    this.backend.get_map_winrate_over_time(this.backend.username).subscribe(data => {
      let maps: Record<string, Record<string, DateData>> = {};
      // Process the data to organize it by map, date, wins, and total
      data.forEach(game => {
        if (!maps[game.map]) {
          maps[game.map] = {};
        }
        if (!maps[game.map][game.date]) {
          maps[game.map][game.date] = { wins: 0, total: 0 };
        }
        if (game.win) {
          maps[game.map][game.date].wins++;
        }
        maps[game.map][game.date].total++;
      });
  
      // Sort dates and accumulate wins and totals for each map
      Object.keys(maps).forEach(mapId => {
        const dates = Object.keys(maps[mapId]).sort();
        let cumulativeWins = 0;
        let cumulativeTotal = 0;
  
        dates.forEach(date => {
          cumulativeWins += maps[mapId][date].wins;
          cumulativeTotal += maps[mapId][date].total;
          // Update the map data with cumulative values
          maps[mapId][date].wins = cumulativeWins;
          maps[mapId][date].total = cumulativeTotal;
        });
      });
  
      let allDates = Array.from(new Set(Object.values(maps).flatMap(map => Object.keys(map)))).sort();

      let series: (LineSeriesOption | BarSeriesOption)[] = Object.keys(maps).map(mapId => {
      let lastKnownWinRate = 0; // Initialize last known win rate
      let dataForSeries = allDates.map(date => {
      if (maps[mapId][date]) {
      // If there's data for this date, calculate the win rate
        lastKnownWinRate = (maps[mapId][date].wins / maps[mapId][date].total) * 100;
      }
      // Use the last known win rate for this date
      return lastKnownWinRate;
    });

    return {
      name: `Map ${mapId}`,
      type: 'line',
      data: dataForSeries,
      stack: 'counts',
      areaStyle: {},
    } as LineSeriesOption;
});


      // Update the chart options
      this.options = {
        tooltip: {
          trigger: 'axis',
          axisPointer: {
            type: 'cross',
            label: {
              backgroundColor: '#6a7985',
            },
          },
        },
        legend: {
          data: Object.keys(maps).map(mapId => `Map ${mapId}`),
        },
        grid: {
          left: '3%',
          right: '4%',
          bottom: '3%',
          containLabel: true,
        },
        xAxis: {
          type: 'category',
          boundaryGap: false,
          data: allDates,
        },
        yAxis: {
          type: 'value',
          axisLabel: {
            formatter: '{value} %' // Format yAxis labels as percentages
          }
        },
        series: series, // This now matches the expected type
      };
    });
  }
  
}
