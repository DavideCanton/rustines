import { Component, inject } from '@angular/core';
import { VERSION } from '../generated/version';
import { EventType, Router, RouterLinkWithHref } from '@angular/router';
import { toSignal } from '@angular/core/rxjs-interop';
import { filter, map } from 'rxjs/operators';

const DISCLAIMER_ROUTE = '/disclaimer';

@Component({
  imports: [RouterLinkWithHref],
  selector: 'rustines-footer',
  styleUrl: './footer.component.scss',
  templateUrl: './footer.component.html',
})
export class FooterComponent  {
  protected readonly version = VERSION;

  protected router = inject(Router);

  protected showDisclaimerLink = toSignal(
    this.router.events.pipe(
      filter((e) => e.type == EventType.NavigationEnd),
      map((e) => e.urlAfterRedirects != DISCLAIMER_ROUTE),
    ),
    { initialValue: this.router.url == DISCLAIMER_ROUTE },
  );
}
