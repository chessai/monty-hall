module Main (main) where

import Control.Monad ((>=>))
import Data.Coerce (coerce)
import Data.List ((\\))
import qualified Numeric.Probability.Distribution as D
import qualified Numeric.Probability.Transition as T

main :: IO ()
main = do
  print (simulate Switch)
  print (simulate Stay)

data Door = DoorOne | DoorTwo | DoorThree
  deriving (Eq, Ord, Enum, Bounded, Show, Read)

doors :: [Door]
doors = [minBound .. maxBound]

data Doors = Doors
  { prize :: Door
  , chosen :: Door
  , opened :: Door
  }
  deriving (Eq, Ord, Show, Read)

newtype Transition = Transition { getTransition :: T.T Double Doors }

hide :: Transition
hide = Transition $ \s -> D.uniform [s { prize = d } | d <- doors ]

choose :: Transition
choose = Transition $ \s -> D.uniform [ s { chosen = d } | d <- doors ]

open :: Transition
open = Transition $ \s -> D.uniform [ s { opened = d } | d <- doors \\ [ prize s, chosen s ]]

data Strategy = Switch | Stay
  deriving (Eq, Ord, Show, Read)

strategise :: Strategy -> Transition
strategise Switch = switch
strategise Stay = stay

switch :: Transition
switch = Transition $ \s -> D.uniform [ s { chosen = d } | d <- doors \\ [ chosen s, opened s ]]

stay :: Transition
stay = Transition T.id

game :: Strategy -> Transition
game s = Transition $ foldl (>=>) T.id allTs where
  allTs :: [T.T Double Doors]
  allTs = coerce [hide, choose, open, strategise s]

data Outcome = Win | Lose
  deriving (Eq, Ord, Show, Read)

result :: Doors -> Result
result s = if chosen s == prize s then Result 1 0 else Result 0 1

simulate :: Strategy -> Result
simulate s = foldMap result
  . D.extract
  . ($ Doors undefined undefined undefined)
  . getTransition
  $ game s

data Result = Result
  { won :: Int
  , lost :: Int
  }
  deriving (Eq, Ord, Show, Read)

instance Semigroup Result where
  Result a b <> Result a' b' = Result (a + a') (b + b')

instance Monoid Result where
  mempty = Result 0 0

