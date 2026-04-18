import { HttpInterceptorFn, HttpErrorResponse, HttpRequest, HttpHandlerFn } from '@angular/common/http';
import { inject } from '@angular/core';
import { catchError, filter, switchMap, take, throwError } from 'rxjs';
import { BehaviorSubject } from 'rxjs';
import { AuthService } from './services/auth.service';
import { environment } from '../environments/environment';

const isRefreshing$ = new BehaviorSubject<boolean>(false);

export const authInterceptor: HttpInterceptorFn = (req, next) => {
    const authService = inject(AuthService);

    const reqWithCreds = req.clone({ withCredentials: true });

    return next(reqWithCreds).pipe(
        catchError((error: HttpErrorResponse) => {
            if (error.status === 401 && !req.url.includes(`${environment.baseUrl}/api/v1/refresh`)) {
                return handle401(reqWithCreds, next, authService);
            }
            return throwError(() => error);
        })
    );
};

function handle401(req: HttpRequest<unknown>, next: HttpHandlerFn, authService: AuthService) {
    if (!isRefreshing$.value) {
        isRefreshing$.next(true);

        return authService.refresh().pipe(
            switchMap(() => {
                isRefreshing$.next(false);
                return next(req);
            }),
            catchError((err) => {
                isRefreshing$.next(false);
                authService.logout();
                return throwError(() => err);
            })
        );
    }

    // Queue requests while refresh is in flight
    return isRefreshing$.pipe(
        filter(refreshing => !refreshing),
        take(1),
        switchMap(() => next(req))
    );
}
