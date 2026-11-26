import {
  ApplicationConfig,
  provideBrowserGlobalErrorListeners,
} from '@angular/core';
import { provideRouter } from '@angular/router';
import { PlayComponent } from './play/play.component';
import { RomChooserComponent } from './rom-chooser/rom-chooser.component';
import { DisclaimerComponent } from './disclaimer/disclaimer.component';

export const appConfig: ApplicationConfig = {
  providers: [
    provideBrowserGlobalErrorListeners(),
    provideRouter([
      {
        path: '',
        redirectTo: '/rom-choose',
        pathMatch: 'full',
      },
      {
        path: 'rom-choose',
        component: RomChooserComponent,
      },
      {
        path: 'play',
        component: PlayComponent,
      },
      {
        path: 'disclaimer',
        component: DisclaimerComponent,
      },
    ]),
  ],
};
