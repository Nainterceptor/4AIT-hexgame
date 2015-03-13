/**
 * Created by Robin on 06/03/2015.
 */
function isOdd(n)
{
    return isNumber(n) && (Math.abs(n) % 2 == 1);
}

function isNumber(n)
{
    return n == parseFloat(n);
}
(function(){

    angular.module('hex',['luegg.directives','Utils','AI','Games'])
        .controller('GameController',['$scope','GridService','InfosService','PlayService',
            function($scope,GridService,InfosService,PlayService){
                me = this;
                me.gridSize=3;
                me.infos=InfosService;
                me.firstPlayer = 'blue';
                me.bluePlayer = 'path';
                me.redPlayer = 'path';

                var init = function(){
                    InfosService.message = "\n---- Logs ----\n\n";
                    GridService.setGridSize(me.gridSize);
                    GridService.makeGrid();
                };

                me.goodToPlay = function(){
                    return isOdd(me.gridSize)&&me.gridSize>0;
                };

                me.start = function(){
                    init();
                    PlayService.play(me.firstPlayer, me.bluePlayer, me.redPlayer);
                };
            }]);
})();