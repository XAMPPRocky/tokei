/*
    Copyright (c) 2012 Torbjörn Klatt <opensource@torbjoern-klatt.de>

    Permission is hereby granted, free of charge, to any person
    obtaining a copy of this software and associated documentation
    files (the "Software"), to deal in the Software without
    restriction, including without limitation the rights to use,
    copy, modify, merge, publish, distribute, sublicense, and/or sell
    copies of the Software, and to permit persons to whom the
    Software is furnished to do so, subject to the following
    conditions:

    The above copyright notice and this permission notice shall be
    included in all copies or substantial portions of the Software.

    THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
    EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES
    OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
    NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT
    HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY,
    WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
    FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
    OTHER DEALINGS IN THE SOFTWARE.
*/

#include "../helper.hpp"

// Do not to forget to rename the preprocessor directives as well!
#ifndef KOANXX_SAMPLE_KOANS_HPP
#define KOANXX_SAMPLE_KOANS_HPP

// Rename the Episode
class KoanXX_sample_koans : Koan
{
  private:
    KoanHandler *status;                //!
    // When ever a koan is added at the very bottom, this counter needs to be
    // increased.
    static const int num_tests = 1;     //!

  public:
    /**
     *
     */
    KoanXX_sample_koans( KoanHandler *status ) : status( status ) {
      status->register_koans( num_tests );
    }
    /**
     *
     */
    ~KoanXX_sample_koans() {}

    /**
     *
     */
    void run() {
      // For each koan in this episode, one line needs to be written.
      // The koans are executed in the order they are called here.
      status->eval_koan( *this, static_cast<void ( Koan:: * )()>( &KoanXX_sample_koans::a_sample_koan ) );

      status->episode_done( "the-so-and-so'th" );
    }

    /**
     *
     */
    static int get_num_tests() {
      return num_tests;
    }

  private:
    // Add further Koans down here by defining their name.
    // The implementation of these is done in ~/koans/koanXX_sample_koans.cpp
    // REMARK: Do not forget to increase this.num_tests when you add another koan
    void a_sample_koan();
};

#endif // KOANXX_SAMPLE_KOANS_HPP

// EOF
